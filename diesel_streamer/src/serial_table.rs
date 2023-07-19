// TODO: Update macro to be more composable for defaults

/// Streams a serial table for diesel schemas. A serial table has an
/// autoincremented field which is used to cursor through the table
/// for processing.
///
///
/// # Examples
///
/// Stream a serial table with default chunk_size, from, and to
///
/// ```ignore
/// async fn main() {
///     use diesel_streamer::stream_serial_table;
///     use crate::schema::some_table::dsl::{some_table, autoincremented_field};
///
///     let mut conn = pool.get().await.unwrap();
///
///     stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     });
///
///     let chunk_size = 20;
///     
///    // with chunk size
///    stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     }, chunk_size);
///    
///    // with a specified beginning
///    let beginning_id = 200;
///    stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     }, chunk_size, beginning_id);
///
///   // with a specified end
///    let end_id = 340;
///    stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     }, chunk_size, beginning_id, end_id);
/// }
/// ```
#[macro_export]
#[cfg(feature = "async")]
#[cfg(not(feature = "sync"))]
macro_rules! stream_serial_table {
    ( $query:expr ,   $cursor_field:expr ,  $conn: expr , $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let default_chunk_size = 500;
        let mut default_from = 0;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            default_chunk_size,
            default_from,
            default_to,
            $stream_processor,
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $stream_processor: expr) => {{
        use crate::stream_serial_table;

        let mut default_from = 0;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            default_from,
            default_to,
            $stream_processor,
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $from: expr, $stream_processor: expr) => {{
        use crate::stream_serial_table;

        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            $from,
            default_to,
            $stream_processor,
        )
    }};

    ( $query:expr ,   $cursor_field:expr ,  $conn: expr ,  $chunk_size:expr , $from:expr, $to:expr, $stream_processor: expr,) => {{
        use diesel::dsl::max;
        use diesel::{prelude::*, QueryDsl, RunQueryDsl};

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
                    .filter($cursor_field.eq_any($from..=chunk_limit))
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
///
/// # Examples
///
/// Stream a serial table with default chunk_size, from, and to
///
/// ```ignore
/// async fn main() {
///     use diesel_streamer::stream_serial_table;
///     use crate::schema::some_table::dsl::{some_table, autoincremented_field};
///
///     let mut conn = pool.get().await.unwrap();
///
///     stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     });
///
///     let chunk_size = 20;
///     
///    // with chunk size
///    stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     }, chunk_size);
///
///    
///    // with a specified beginning
///    let beginning_id = 200;
///    stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     }, chunk_size, beginning_id);
///
///   // with a specified end
///    let end_id = 340;
///    stream_serial_table!(some_table, autoincremented_field, conn, |streamed_table_data| async move {
///         // do work here
///     }, chunk_size, beginning_id, end_id);
/// }
/// ```
#[macro_export]
#[cfg(feature = "sync")]
#[cfg(not(feature = "async"))]
macro_rules! stream_serial_table {
    ( $query:expr ,   $cursor_field:expr ,  $conn: expr , $stream_processor: expr) => {{
        use diesel_streamer::stream_serial_table;

        let default_chunk_size = 100000;
        let mut default_from = 0;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            default_chunk_size,
            default_from,
            default_to,
            $stream_processor,
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $stream_processor: expr) => {{
        use crate::stream_serial_table;

        let mut default_from = 0;
        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            default_from,
            default_to,
            $stream_processor,
        )
    }};

    ( $query:expr , $cursor_field:expr , $conn: expr , $chunk_size: expr, $from: expr, $stream_processor: expr) => {{
        use crate::stream_serial_table;

        let default_to = None;

        stream_serial_table!(
            $query,
            $cursor_field,
            $conn,
            $chunk_size,
            $from,
            default_to,
            $stream_processor,
        )
    }};

    ( $query:expr ,   $cursor_field:expr ,  $conn: expr ,  $chunk_size:expr , $from:expr, $to:expr, $stream_processor: expr,) => {{
        use diesel::dsl::max;
        use diesel::{prelude::*, QueryDsl, RunQueryDsl};

        let to = match $to {
            Some(to) => to,
            None => $query
                .select(max($cursor_field))
                .get_result::<Option<i32>>($conn)
                .unwrap()
                .unwrap_or(0),
        };

        if (to > $from) {
            while $from <= to {
                let chunk_limit = $from + $chunk_size;

                let streamed_data = $query
                    .filter($cursor_field.eq_any($from..=chunk_limit))
                    .load($conn)
                    .unwrap();

                ($stream_processor)(streamed_data);

                $from = chunk_limit;
            }
        }
    }};
}
