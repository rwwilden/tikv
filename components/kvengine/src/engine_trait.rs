// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use crate::*;
use engine_traits::{
    CFNamesExt, CFOptionsExt, ColumnFamilyOptions, CompactExt, CompactedEvent, DBOptions,
    DBOptionsExt, DBVector, DecodeProperties, DeleteStrategy, ExternalSstFileInfo, ImportExt,
    IngestExternalFileOptions, IterOptions, Iterable, KvEngine, MiscExt, Mutable, MvccProperties,
    MvccPropertiesExt, Peekable, PerfContext, PerfContextExt, PerfContextKind, PerfLevel, Range,
    RangePropertiesExt, ReadOptions, SeekKey, Snapshot, SstCompressionType, SstExt,
    SstPartitionerFactory, SstReader, SstWriter, SstWriterBuilder, SyncMutable, TableProperties,
    TablePropertiesCollection, TablePropertiesCollectionIter, TablePropertiesExt,
    TablePropertiesKey, TitanDBOptions, TtlProperties, TtlPropertiesExt, UserCollectedProperties,
    WriteBatchExt, WriteOptions,
};
use std::collections::BTreeMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};

type TraitsResult<T> = std::result::Result<T, engine_traits::Error>;

impl CFNamesExt for Engine {
    fn cf_names(&self) -> Vec<&str> {
        panic!()
    }
}

impl CFOptionsExt for Engine {
    type ColumnFamilyOptions = EngineColumnFamilyOptions;

    fn get_options_cf(&self, cf: &str) -> TraitsResult<Self::ColumnFamilyOptions> {
        panic!()
    }
    fn set_options_cf(&self, cf: &str, options: &[(&str, &str)]) -> TraitsResult<()> {
        panic!()
    }
}

pub struct EngineColumnFamilyOptions;

impl ColumnFamilyOptions for EngineColumnFamilyOptions {
    type TitanDBOptions = EngineTitanDBOptions;

    fn new() -> Self {
        panic!()
    }
    fn get_max_write_buffer_number(&self) -> u32 {
        panic!()
    }
    fn get_level_zero_slowdown_writes_trigger(&self) -> u32 {
        panic!()
    }
    fn get_level_zero_stop_writes_trigger(&self) -> u32 {
        panic!()
    }
    fn set_level_zero_file_num_compaction_trigger(&mut self, v: i32) {
        panic!()
    }
    fn get_soft_pending_compaction_bytes_limit(&self) -> u64 {
        panic!()
    }
    fn get_hard_pending_compaction_bytes_limit(&self) -> u64 {
        panic!()
    }
    fn get_block_cache_capacity(&self) -> u64 {
        panic!()
    }
    fn set_block_cache_capacity(&self, capacity: u64) -> std::result::Result<(), String> {
        panic!()
    }
    fn set_titandb_options(&mut self, opts: &Self::TitanDBOptions) {
        panic!()
    }
    fn get_target_file_size_base(&self) -> u64 {
        panic!()
    }
    fn set_disable_auto_compactions(&mut self, v: bool) {
        panic!()
    }
    fn get_disable_auto_compactions(&self) -> bool {
        panic!()
    }
    fn set_sst_partitioner_factory<F: SstPartitionerFactory>(&mut self, factory: F) {
        panic!()
    }
}

impl CompactExt for Engine {
    type CompactedEvent = EngineCompactedEvent;

    fn auto_compactions_is_disabled(&self) -> TraitsResult<bool> {
        panic!()
    }

    fn compact_range(
        &self,
        cf: &str,
        start_key: Option<&[u8]>,
        end_key: Option<&[u8]>,
        exclusive_manual: bool,
        max_subcompactions: u32,
    ) -> TraitsResult<()> {
        panic!()
    }

    fn compact_files_in_range(
        &self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        output_level: Option<i32>,
    ) -> TraitsResult<()> {
        panic!()
    }

    fn compact_files_in_range_cf(
        &self,
        cf: &str,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        output_level: Option<i32>,
    ) -> TraitsResult<()> {
        panic!()
    }

    fn compact_files_cf(
        &self,
        cf: &str,
        files: Vec<String>,
        output_level: Option<i32>,
        max_subcompactions: u32,
        exclude_l0: bool,
    ) -> TraitsResult<()> {
        panic!()
    }
}

pub struct EngineCompactedEvent;

impl CompactedEvent for EngineCompactedEvent {
    fn total_bytes_declined(&self) -> u64 {
        panic!()
    }

    fn is_size_declining_trivial(&self, split_check_diff: u64) -> bool {
        panic!()
    }

    fn output_level_label(&self) -> String {
        panic!()
    }

    fn calc_ranges_declined_bytes(
        self,
        ranges: &BTreeMap<Vec<u8>, u64>,
        bytes_threshold: u64,
    ) -> Vec<(u64, u64)> {
        panic!()
    }

    fn cf(&self) -> &str {
        panic!()
    }
}

impl DBOptionsExt for Engine {
    type DBOptions = EngineDBOptions;

    fn get_db_options(&self) -> Self::DBOptions {
        panic!()
    }
    fn set_db_options(&self, options: &[(&str, &str)]) -> TraitsResult<()> {
        panic!()
    }
}

pub struct EngineDBOptions;

impl DBOptions for EngineDBOptions {
    type TitanDBOptions = EngineTitanDBOptions;

    fn new() -> Self {
        panic!()
    }

    fn get_max_background_jobs(&self) -> i32 {
        panic!()
    }

    fn get_rate_bytes_per_sec(&self) -> Option<i64> {
        panic!()
    }

    fn set_rate_bytes_per_sec(&mut self, rate_bytes_per_sec: i64) -> TraitsResult<()> {
        panic!()
    }

    fn get_rate_limiter_auto_tuned(&self) -> Option<bool> {
        panic!()
    }

    fn set_rate_limiter_auto_tuned(&mut self, rate_limiter_auto_tuned: bool) -> TraitsResult<()> {
        panic!()
    }

    fn set_titandb_options(&mut self, opts: &Self::TitanDBOptions) {
        panic!()
    }
}

pub struct EngineTitanDBOptions;

impl TitanDBOptions for EngineTitanDBOptions {
    fn new() -> Self {
        panic!()
    }
    fn set_min_blob_size(&mut self, size: u64) {
        panic!()
    }
}

#[derive(Debug)]
pub struct EngineDBVector;

impl DBVector for EngineDBVector {}

impl Deref for EngineDBVector {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        panic!()
    }
}

impl<'a> PartialEq<&'a [u8]> for EngineDBVector {
    fn eq(&self, rhs: &&[u8]) -> bool {
        **rhs == **self
    }
}

impl KvEngine for Engine {
    type Snapshot = EngineSnapshot;

    fn snapshot(&self) -> Self::Snapshot {
        panic!()
    }
    fn sync(&self) -> TraitsResult<()> {
        panic!()
    }
    fn bad_downcast<T: 'static>(&self) -> &T {
        panic!()
    }
}

impl Peekable for Engine {
    type DBVector = EngineDBVector;

    fn get_value_opt(
        &self,
        opts: &ReadOptions,
        key: &[u8],
    ) -> TraitsResult<Option<Self::DBVector>> {
        panic!()
    }
    fn get_value_cf_opt(
        &self,
        opts: &ReadOptions,
        cf: &str,
        key: &[u8],
    ) -> TraitsResult<Option<Self::DBVector>> {
        panic!()
    }
}

impl SyncMutable for Engine {
    fn put(&self, key: &[u8], value: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn put_cf(&self, cf: &str, key: &[u8], value: &[u8]) -> TraitsResult<()> {
        panic!()
    }

    fn delete(&self, key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn delete_cf(&self, cf: &str, key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn delete_range(&self, begin_key: &[u8], end_key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn delete_range_cf(&self, cf: &str, begin_key: &[u8], end_key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
}

impl Iterable for Engine {
    type Iterator = EngineIterator;

    fn iterator_opt(&self, opts: IterOptions) -> TraitsResult<Self::Iterator> {
        panic!()
    }
    fn iterator_cf_opt(&self, cf: &str, opts: IterOptions) -> TraitsResult<Self::Iterator> {
        panic!()
    }
}

pub struct EngineIterator;

impl engine_traits::Iterator for EngineIterator {
    fn seek(&mut self, key: SeekKey) -> TraitsResult<bool> {
        panic!()
    }
    fn seek_for_prev(&mut self, key: SeekKey) -> TraitsResult<bool> {
        panic!()
    }

    fn prev(&mut self) -> TraitsResult<bool> {
        panic!()
    }
    fn next(&mut self) -> TraitsResult<bool> {
        panic!()
    }

    fn key(&self) -> &[u8] {
        panic!()
    }
    fn value(&self) -> &[u8] {
        panic!()
    }

    fn valid(&self) -> TraitsResult<bool> {
        panic!()
    }
}

impl ImportExt for Engine {
    type IngestExternalFileOptions = EngineIngestExternalFileOptions;

    fn ingest_external_file_cf(
        &self,
        cf: &str,
        opts: &Self::IngestExternalFileOptions,
        files: &[&str],
    ) -> TraitsResult<()> {
        panic!()
    }

    fn reset_global_seq<P: AsRef<Path>>(&self, cf: &str, path: P) -> TraitsResult<()> {
        panic!()
    }
}

pub struct EngineIngestExternalFileOptions;

impl IngestExternalFileOptions for EngineIngestExternalFileOptions {
    fn new() -> Self {
        panic!()
    }
    fn move_files(&mut self, f: bool) {
        panic!()
    }
}

impl MiscExt for Engine {
    fn flush(&self, sync: bool) -> TraitsResult<()> {
        panic!()
    }

    fn flush_cf(&self, cf: &str, sync: bool) -> TraitsResult<()> {
        panic!()
    }

    fn delete_ranges_cf(
        &self,
        cf: &str,
        strategy: DeleteStrategy,
        ranges: &[Range],
    ) -> TraitsResult<()> {
        panic!()
    }

    fn get_approximate_memtable_stats_cf(
        &self,
        cf: &str,
        range: &Range,
    ) -> TraitsResult<(u64, u64)> {
        panic!()
    }

    fn ingest_maybe_slowdown_writes(&self, cf: &str) -> TraitsResult<bool> {
        panic!()
    }

    fn get_engine_used_size(&self) -> TraitsResult<u64> {
        panic!()
    }

    fn roughly_cleanup_ranges(&self, ranges: &[(Vec<u8>, Vec<u8>)]) -> TraitsResult<()> {
        panic!()
    }

    fn path(&self) -> &str {
        panic!()
    }

    fn sync_wal(&self) -> TraitsResult<()> {
        panic!()
    }

    fn exists(path: &str) -> bool {
        panic!()
    }

    fn dump_stats(&self) -> TraitsResult<String> {
        panic!()
    }

    fn get_latest_sequence_number(&self) -> u64 {
        panic!()
    }

    fn get_oldest_snapshot_sequence_number(&self) -> Option<u64> {
        panic!()
    }

    fn get_total_sst_files_size_cf(&self, cf: &str) -> TraitsResult<Option<u64>> {
        panic!()
    }

    fn get_range_entries_and_versions(
        &self,
        cf: &str,
        start: &[u8],
        end: &[u8],
    ) -> TraitsResult<Option<(u64, u64)>> {
        panic!()
    }

    fn get_cf_num_files_at_level(&self, cf: &str, level: usize) -> TraitsResult<Option<u64>> {
        panic!()
    }

    fn get_cf_num_immutable_mem_table(&self, cf: &str) -> TraitsResult<Option<u64>> {
        panic!()
    }

    fn get_cf_compaction_pending_bytes(&self, cf: &str) -> TraitsResult<Option<u64>> {
        panic!()
    }

    fn is_stalled_or_stopped(&self) -> bool {
        panic!()
    }
}

impl MvccPropertiesExt for Engine {
    fn get_mvcc_properties_cf(
        &self,
        cf: &str,
        safe_point: txn_types::TimeStamp,
        start_key: &[u8],
        end_key: &[u8],
    ) -> Option<MvccProperties> {
        panic!()
    }
}

impl PerfContextExt for Engine {
    type PerfContext = EnginePerfContext;

    fn get_perf_context(&self, level: PerfLevel, kind: PerfContextKind) -> Self::PerfContext {
        panic!()
    }
}

pub struct EnginePerfContext;

impl PerfContext for EnginePerfContext {
    fn start_observe(&mut self) {
        panic!()
    }

    fn report_metrics(&mut self) {
        panic!()
    }
}

impl RangePropertiesExt for Engine {
    fn get_range_approximate_keys(&self, range: Range, large_threshold: u64) -> TraitsResult<u64> {
        panic!()
    }

    fn get_range_approximate_keys_cf(
        &self,
        cfname: &str,
        range: Range,
        large_threshold: u64,
    ) -> TraitsResult<u64> {
        panic!()
    }

    fn get_range_approximate_size(&self, range: Range, large_threshold: u64) -> TraitsResult<u64> {
        panic!()
    }

    fn get_range_approximate_size_cf(
        &self,
        cfname: &str,
        range: Range,
        large_threshold: u64,
    ) -> TraitsResult<u64> {
        panic!()
    }

    fn get_range_approximate_split_keys(
        &self,
        range: Range,
        key_count: usize,
    ) -> TraitsResult<Vec<Vec<u8>>> {
        panic!()
    }

    fn get_range_approximate_split_keys_cf(
        &self,
        cfname: &str,
        range: Range,
        key_count: usize,
    ) -> TraitsResult<Vec<Vec<u8>>> {
        panic!()
    }
}

#[derive(Clone, Debug)]
pub struct EngineSnapshot;

impl Snapshot for EngineSnapshot {
    fn cf_names(&self) -> Vec<&str> {
        panic!()
    }
}

impl Peekable for EngineSnapshot {
    type DBVector = EngineDBVector;

    fn get_value_opt(
        &self,
        opts: &ReadOptions,
        key: &[u8],
    ) -> TraitsResult<Option<Self::DBVector>> {
        panic!()
    }
    fn get_value_cf_opt(
        &self,
        opts: &ReadOptions,
        cf: &str,
        key: &[u8],
    ) -> TraitsResult<Option<Self::DBVector>> {
        panic!()
    }
}

impl Iterable for EngineSnapshot {
    type Iterator = PanicSnapshotIterator;

    fn iterator_opt(&self, opts: IterOptions) -> TraitsResult<Self::Iterator> {
        panic!()
    }
    fn iterator_cf_opt(&self, cf: &str, opts: IterOptions) -> TraitsResult<Self::Iterator> {
        panic!()
    }
}

pub struct PanicSnapshotIterator;

impl engine_traits::Iterator for PanicSnapshotIterator {
    fn seek(&mut self, key: SeekKey) -> TraitsResult<bool> {
        panic!()
    }
    fn seek_for_prev(&mut self, key: SeekKey) -> TraitsResult<bool> {
        panic!()
    }

    fn prev(&mut self) -> TraitsResult<bool> {
        panic!()
    }
    fn next(&mut self) -> TraitsResult<bool> {
        panic!()
    }

    fn key(&self) -> &[u8] {
        panic!()
    }
    fn value(&self) -> &[u8] {
        panic!()
    }

    fn valid(&self) -> TraitsResult<bool> {
        panic!()
    }
}

impl SstExt for Engine {
    type SstReader = EngineSstReader;
    type SstWriter = EngineSstWriter;
    type SstWriterBuilder = EngineSstWriterBuilder;
}

pub struct EngineSstReader;

impl SstReader for EngineSstReader {
    fn open(path: &str) -> TraitsResult<Self> {
        panic!()
    }
    fn verify_checksum(&self) -> TraitsResult<()> {
        panic!()
    }
    fn iter(&self) -> Self::Iterator {
        panic!()
    }
}

impl Iterable for EngineSstReader {
    type Iterator = EngineSstReaderIterator;

    fn iterator_opt(&self, opts: IterOptions) -> TraitsResult<Self::Iterator> {
        panic!()
    }
    fn iterator_cf_opt(&self, cf: &str, opts: IterOptions) -> TraitsResult<Self::Iterator> {
        panic!()
    }
}

pub struct EngineSstReaderIterator;

impl engine_traits::Iterator for EngineSstReaderIterator {
    fn seek(&mut self, key: SeekKey) -> TraitsResult<bool> {
        panic!()
    }
    fn seek_for_prev(&mut self, key: SeekKey) -> TraitsResult<bool> {
        panic!()
    }

    fn prev(&mut self) -> TraitsResult<bool> {
        panic!()
    }
    fn next(&mut self) -> TraitsResult<bool> {
        panic!()
    }

    fn key(&self) -> &[u8] {
        panic!()
    }
    fn value(&self) -> &[u8] {
        panic!()
    }

    fn valid(&self) -> TraitsResult<bool> {
        panic!()
    }
}

pub struct EngineSstWriter;

impl SstWriter for EngineSstWriter {
    type ExternalSstFileInfo = EngineExternalSstFileInfo;
    type ExternalSstFileReader = EngineExternalSstFileReader;

    fn put(&mut self, key: &[u8], val: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn delete(&mut self, key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn file_size(&mut self) -> u64 {
        panic!()
    }
    fn finish(self) -> TraitsResult<Self::ExternalSstFileInfo> {
        panic!()
    }
    fn finish_read(self) -> TraitsResult<(Self::ExternalSstFileInfo, Self::ExternalSstFileReader)> {
        panic!()
    }
}

pub struct EngineSstWriterBuilder;

impl SstWriterBuilder<Engine> for EngineSstWriterBuilder {
    fn new() -> Self {
        panic!()
    }
    fn set_db(self, db: &Engine) -> Self {
        panic!()
    }
    fn set_cf(self, cf: &str) -> Self {
        panic!()
    }
    fn set_in_memory(self, in_memory: bool) -> Self {
        panic!()
    }
    fn set_compression_type(self, compression: Option<SstCompressionType>) -> Self {
        panic!()
    }
    fn set_compression_level(self, level: i32) -> Self {
        panic!()
    }

    fn build(self, path: &str) -> TraitsResult<EngineSstWriter> {
        panic!()
    }
}

pub struct EngineExternalSstFileInfo;

impl ExternalSstFileInfo for EngineExternalSstFileInfo {
    fn new() -> Self {
        panic!()
    }
    fn file_path(&self) -> PathBuf {
        panic!()
    }
    fn smallest_key(&self) -> &[u8] {
        panic!()
    }
    fn largest_key(&self) -> &[u8] {
        panic!()
    }
    fn sequence_number(&self) -> u64 {
        panic!()
    }
    fn file_size(&self) -> u64 {
        panic!()
    }
    fn num_entries(&self) -> u64 {
        panic!()
    }
}

pub struct EngineExternalSstFileReader;

impl std::io::Read for EngineExternalSstFileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        panic!()
    }
}

impl TablePropertiesExt for Engine {
    type TablePropertiesCollection = EngineTablePropertiesCollection;
    type TablePropertiesCollectionIter = EngineTablePropertiesCollectionIter;
    type TablePropertiesKey = EngineTablePropertiesKey;
    type TableProperties = EngineTableProperties;
    type UserCollectedProperties = EngineUserCollectedProperties;

    fn get_properties_of_tables_in_range(
        &self,
        cf: &str,
        ranges: &[Range],
    ) -> TraitsResult<Self::TablePropertiesCollection> {
        panic!()
    }
}

pub struct EngineTablePropertiesCollection;

impl
    TablePropertiesCollection<
        EngineTablePropertiesCollectionIter,
        EngineTablePropertiesKey,
        EngineTableProperties,
        EngineUserCollectedProperties,
    > for EngineTablePropertiesCollection
{
    fn iter(&self) -> EngineTablePropertiesCollectionIter {
        panic!()
    }

    fn len(&self) -> usize {
        panic!()
    }
}

pub struct EngineTablePropertiesCollectionIter;

impl
    TablePropertiesCollectionIter<
        EngineTablePropertiesKey,
        EngineTableProperties,
        EngineUserCollectedProperties,
    > for EngineTablePropertiesCollectionIter
{
}

impl core::iter::Iterator for EngineTablePropertiesCollectionIter {
    type Item = (EngineTablePropertiesKey, EngineTableProperties);

    fn next(&mut self) -> Option<Self::Item> {
        panic!()
    }
}

pub struct EngineTablePropertiesKey;

impl TablePropertiesKey for EngineTablePropertiesKey {}

impl Deref for EngineTablePropertiesKey {
    type Target = str;

    fn deref(&self) -> &str {
        panic!()
    }
}

pub struct EngineTableProperties;

impl TableProperties<EngineUserCollectedProperties> for EngineTableProperties {
    fn num_entries(&self) -> u64 {
        panic!()
    }

    fn user_collected_properties(&self) -> EngineUserCollectedProperties {
        panic!()
    }
}

pub struct EngineUserCollectedProperties;

impl UserCollectedProperties for EngineUserCollectedProperties {
    fn get(&self, index: &[u8]) -> Option<&[u8]> {
        panic!()
    }

    fn len(&self) -> usize {
        panic!()
    }
}

impl DecodeProperties for EngineUserCollectedProperties {
    fn decode(&self, k: &str) -> tikv_util::codec::Result<&[u8]> {
        panic!()
    }
}

impl TtlPropertiesExt for Engine {
    fn get_range_ttl_properties_cf(
        &self,
        cf: &str,
        start_key: &[u8],
        end_key: &[u8],
    ) -> TraitsResult<Vec<(String, TtlProperties)>> {
        panic!()
    }
}

impl WriteBatchExt for Engine {
    type WriteBatch = EngineWriteBatch;
    type WriteBatchVec = EngineWriteBatch;

    const WRITE_BATCH_MAX_KEYS: usize = 1;

    fn support_write_batch_vec(&self) -> bool {
        panic!()
    }

    fn write_batch(&self) -> Self::WriteBatch {
        panic!()
    }
    fn write_batch_with_cap(&self, cap: usize) -> Self::WriteBatch {
        panic!()
    }
}

pub struct EngineWriteBatch;

impl engine_traits::WriteBatch<Engine> for EngineWriteBatch {
    fn with_capacity(_: &Engine, _: usize) -> Self {
        panic!()
    }

    fn write_opt(&self, _: &WriteOptions) -> TraitsResult<()> {
        panic!()
    }

    fn data_size(&self) -> usize {
        panic!()
    }
    fn count(&self) -> usize {
        panic!()
    }
    fn is_empty(&self) -> bool {
        panic!()
    }
    fn should_write_to_engine(&self) -> bool {
        panic!()
    }

    fn clear(&mut self) {
        panic!()
    }
    fn set_save_point(&mut self) {
        panic!()
    }
    fn pop_save_point(&mut self) -> TraitsResult<()> {
        panic!()
    }
    fn rollback_to_save_point(&mut self) -> TraitsResult<()> {
        panic!()
    }
}

impl Mutable for EngineWriteBatch {
    fn put(&mut self, key: &[u8], value: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn put_cf(&mut self, cf: &str, key: &[u8], value: &[u8]) -> TraitsResult<()> {
        panic!()
    }

    fn delete(&mut self, key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn delete_cf(&mut self, cf: &str, key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn delete_range(&mut self, begin_key: &[u8], end_key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
    fn delete_range_cf(&mut self, cf: &str, begin_key: &[u8], end_key: &[u8]) -> TraitsResult<()> {
        panic!()
    }
}
