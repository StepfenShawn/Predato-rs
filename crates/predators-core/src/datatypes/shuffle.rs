/*
 message ShuffleId {
   string job_uuid = 1;
   uint32 stage_id = 2;
   uint32 partition_id = 4;
 }
*/
struct ShuffleId {
    job_uuid: String,
    stage_id: u32,
    partition_id: u32,
}

/*
 message ShuffleLocation {
   string job_uuid = 1;
   uint32 stage_id = 2;
   uint32 partition_id = 4;
   string executor_uuid = 5;
 }
*/

struct ShuffleLocation {
    job_uuid: String,
    stage_id: u32,
    paertition_id: u32,
    executor_uuid: String,
}
