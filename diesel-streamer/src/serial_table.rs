/// Streams a serial table for diesel schemas. A serial table has an
/// autoincremented field which is used to cursor through the table
/// for processing.
///
/// Defaults:
///
/// - Chunk size is 500
/// - Cursor's beginning is the minimum value of the serial table
/// - Cursor's end is the maximum value of the serial table
///
///
/// # Examples
///
/// Stream a serial table using different configurations.
///
/// ```ignore
/// use diesel_streamer::stream_serial_table;
///
/// async fn main() {
///     use crate::schema::some_table::dsl::{some_table, serial_field};
///
///     let mut conn = pool.get().await.unwrap();
///
///     stream_serial_table!(some_table, serial_field, conn, |streamed_table_data| async {
///         // do work here
///     });
///
///     let chunk_size = 20;
///     
///    // with chunk size
///    stream_serial_table!(some_table, serial_field, conn, chunk_size, |streamed_table_data| async {
///         // do work here
///     });
///    
///    // with a specified beginning
///    let beginning_id = Some(200);
///    stream_serial_table!(some_table, serial_field, conn,  chunk_size, beginning_id, |streamed_table_data| async {
///         // do work here
///     });
///
///   // with a specified end
///    let end_id = Some(340);
///    stream_serial_table!(some_table, serial_field, conn,  chunk_size, beginning_id, end_id, |streamed_table_data| async {
///         // do work here
///     });
/// }
/// ```
#[macro_export]
#[cfg(feature = "async")]
macro_rules! stream_serial_table {
    ( $query:expr ,   $cursor_field:expr ,  $conn: expr , $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let default_chunk_size = 500;
        let default_from = None;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            default_chunk_size,
            default_from,
            default_to,
            $stream_processor
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let default_from = None;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            default_from,
            default_to,
            $stream_processor
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $from: expr, $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            $from,
            default_to,
            $stream_processor
        )
    }};

    ( $query:expr ,   $cursor_field:expr ,  $conn: expr ,  $chunk_size:expr , $from:expr, $to:expr, $stream_processor: expr) => {{
        use diesel::dsl::{max, min};
        use diesel::{prelude::*, QueryDsl, RunQueryDsl};

        let mut from = match $from {
            Some(from) => from,
            None => $query
                .select(min($cursor_field))
                .get_result::<Option<i32>>($conn)
                .unwrap()
                .unwrap_or(0),
        };

        let to = match $to {
            Some(to) => to,
            None => $query
                .select(max($cursor_field))
                .get_result::<Option<i32>>($conn)
                .await
                .unwrap()
                .unwrap_or(0),
        };

        if (to > $from) {
            while $from <= to {
                let chunk_limit = $from + $chunk_size;

                let streamed_data = $query
                    .filter($cursor_field.eq_any($from..chunk_limit))
                    .load($conn)
                    .await
                    .unwrap();

                ($stream_processor)(streamed_data).await;

                $from = chunk_limit;
            }
        }
    }};
}

/// Streams a serial table for diesel schemas. A serial table has an
/// autoincremented field which is used to cursor through the table
/// for processing.
///
/// Defaults:
///
/// - Chunk size is 500
/// - Cursor's beginning is the minimum value of the serial table
/// - Cursor's end is the maximum value of the serial table
///
/// # Examples
///
/// Stream a serial table with default chunk_size, from, and to
///
/// ```ignore
/// use diesel_streamer::stream_serial_table;
///
///
/// async fn main() {
///     use crate::schema::some_table::dsl::{some_table, serial_field};
///
///     let mut conn = pool.get().await.unwrap();
///
///     stream_serial_table!(some_table, serial_field, conn, |streamed_table_data| {
///         // do work here
///     });
///
///     let chunk_size = 20;
///     
///    // with chunk size
///    stream_serial_table!(some_table, serial_field, conn, chunk_size, |streamed_table_data| {
///         // do work here
///     });
///
///    
///    // with a specified beginning
///    let beginning_id = Some(200);
///    stream_serial_table!(some_table, serial_field, conn, chunk_size, beginning_id, |streamed_table_data| {
///         // do work here
///     });
///
///   // with a specified end
///    let end_id = Some(340);
///    stream_serial_table!(some_table, serial_field, conn, chunk_size, beginning_id, end_id, |streamed_table_data| {
///         // do work here
///     });
/// }
/// ```
#[macro_export]
#[cfg(feature = "sync")]
macro_rules! stream_serial_table {
    ( $query:expr ,   $cursor_field:expr ,  $conn: expr , $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let default_chunk_size = 100000;
        let default_from = None;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            default_chunk_size,
            default_from,
            default_to,
            $stream_processor
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let mut default_from = None;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            default_from,
            default_to,
            $stream_processor
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $from: expr, $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            $from,
            default_to,
            $stream_processor
        )
    }};

    ( $query:expr , $cursor_field:expr ,  $conn: expr ,  $chunk_size:expr , $from:expr, $to:expr, $stream_processor: expr) => {{
        use diesel::dsl::{max, min};
        use diesel::{prelude::*, QueryDsl, RunQueryDsl};

        let mut from = match $from {
            Some(from) => from,
            None => $query
                .select(min($cursor_field))
                .get_result::<Option<i32>>($conn)
                .unwrap()
                .unwrap_or(0),
        };

        let to = match $to {
            Some(to) => to,
            None => $query
                .select(max($cursor_field))
                .get_result::<Option<i32>>($conn)
                .unwrap()
                .unwrap_or(0),
        };

        if (to > from) {
            while from <= to {
                let chunk_limit = from + ($chunk_size as i32);

                let streamed_data = $query
                    .filter($cursor_field.eq_any(from..chunk_limit))
                    .load($conn)
                    .unwrap();

                ($stream_processor)(streamed_data);

                from = chunk_limit;
            }
        }
    }};
}
