// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	pub fn rte_pmd_bnxt_get_vf_rx_status(port: u16, vf_id: u16) -> c_int;
	pub fn rte_pmd_bnxt_get_vf_stats(port: u16, vf_id: u16, stats: *mut rte_eth_stats) -> c_int;
	pub fn rte_pmd_bnxt_get_vf_tx_drop_count(port: u16, vf_id: u16, count: *mut u64) -> c_int;
	pub fn rte_pmd_bnxt_mac_addr_add(port: u16, mac_addr: *mut ether_addr, vf_id: u32) -> c_int;
	pub fn rte_pmd_bnxt_reset_vf_stats(port: u16, vf_id: u16) -> c_int;
	pub fn rte_pmd_bnxt_set_all_queues_drop_en(port: u16, on: u8) -> c_int;
	pub fn rte_pmd_bnxt_set_tx_loopback(port: u16, on: u8) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_mac_addr(port: u16, vf: u16, mac_addr: *mut ether_addr) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_mac_anti_spoof(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_persist_stats(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_rate_limit(port: u16, vf: u16, tx_rate: u16, q_msk: u64) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_rxmode(port: u16, vf: u16, rx_mask: u16, on: u8) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_vlan_anti_spoof(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_vlan_filter(port: u16, vlan: u16, vf_mask: u64, vlan_on: u8) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_vlan_insert(port: u16, vf: u16, vlan_id: u16) -> c_int;
	pub fn rte_pmd_bnxt_set_vf_vlan_stripq(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_add_vf_mac_addr(port: u16, vf_id: u16, mac_addr: *mut ether_addr) -> c_int;
	pub fn rte_pmd_i40e_cfg_hash_inset(port: u16, pctype: u64, inset: u64) -> c_int;
	pub fn rte_pmd_i40e_flow_add_del_packet_template(port: u16, conf: *const rte_pmd_i40e_pkt_template_conf, add: u8) -> c_int;
	pub fn rte_pmd_i40e_flow_type_mapping_get(port: u16, mapping_items: *mut rte_pmd_i40e_flow_type_mapping) -> c_int;
	pub fn rte_pmd_i40e_flow_type_mapping_reset(port: u16) -> c_int;
	pub fn rte_pmd_i40e_flow_type_mapping_update(port: u16, mapping_items: *mut rte_pmd_i40e_flow_type_mapping, count: u16, exclusive: u8) -> c_int;
	pub fn rte_pmd_i40e_get_ddp_info(pkg: *mut u8, pkg_size: u32, info: *mut u8, size: u32, type_: rte_pmd_i40e_package_info) -> c_int;
	pub fn rte_pmd_i40e_get_ddp_list(port: u16, buff: *mut u8, size: u32) -> c_int;
	pub fn rte_pmd_i40e_get_vf_stats(port: u16, vf_id: u16, stats: *mut rte_eth_stats) -> c_int;
	pub fn rte_pmd_i40e_inset_get(port: u16, pctype: u8, inset: *mut rte_pmd_i40e_inset, inset_type: rte_pmd_i40e_inset_type) -> c_int;
	pub fn rte_pmd_i40e_inset_set(port: u16, pctype: u8, inset: *mut rte_pmd_i40e_inset, inset_type: rte_pmd_i40e_inset_type) -> c_int;
	pub fn rte_pmd_i40e_ping_vfs(port: u16, vf: u16) -> c_int;
	pub fn rte_pmd_i40e_process_ddp_package(port: u16, buff: *mut u8, size: u32, op: rte_pmd_i40e_package_op) -> c_int;
	pub fn rte_pmd_i40e_ptype_mapping_get(port: u16, mapping_items: *mut rte_pmd_i40e_ptype_mapping, size: u16, count: *mut u16, valid_only: u8) -> c_int;
	pub fn rte_pmd_i40e_ptype_mapping_replace(port: u16, target: u32, mask: u8, pkt_type: u32) -> c_int;
	pub fn rte_pmd_i40e_ptype_mapping_reset(port: u16) -> c_int;
	pub fn rte_pmd_i40e_ptype_mapping_update(port: u16, mapping_items: *mut rte_pmd_i40e_ptype_mapping, count: u16, exclusive: u8) -> c_int;
	pub fn rte_pmd_i40e_query_vfid_by_mac(port: u16, vf_mac: *const ether_addr) -> c_int;
	pub fn rte_pmd_i40e_reset_vf_stats(port: u16, vf_id: u16) -> c_int;
	pub fn rte_pmd_i40e_rss_queue_region_conf(port_id: u16, op_type: rte_pmd_i40e_queue_region_op, arg: *mut c_void) -> c_int;
	pub fn rte_pmd_i40e_set_tc_strict_prio(port: u16, tc_map: u8) -> c_int;
	pub fn rte_pmd_i40e_set_tx_loopback(port: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_broadcast(port: u16, vf_id: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_mac_addr(port: u16, vf_id: u16, mac_addr: *mut ether_addr) -> c_int;
	pub fn rte_pmd_i40e_set_vf_mac_anti_spoof(port: u16, vf_id: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_max_bw(port: u16, vf_id: u16, bw: u32) -> c_int;
	pub fn rte_pmd_i40e_set_vf_multicast_promisc(port: u16, vf_id: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_tc_bw_alloc(port: u16, vf_id: u16, tc_num: u8, bw_weight: *mut u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_tc_max_bw(port: u16, vf_id: u16, tc_no: u8, bw: u32) -> c_int;
	pub fn rte_pmd_i40e_set_vf_unicast_promisc(port: u16, vf_id: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_vlan_anti_spoof(port: u16, vf_id: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_vlan_filter(port: u16, vlan_id: u16, vf_mask: u64, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_vlan_insert(port: u16, vf_id: u16, vlan_id: u16) -> c_int;
	pub fn rte_pmd_i40e_set_vf_vlan_stripq(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_i40e_set_vf_vlan_tag(port: u16, vf_id: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_event_show(port: u16, event: u32, state: *mut u32) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_event_store(port: u16, event: u32, state: u32) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_init(port: u16) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_state_set(port: u16, new_state: *mut u32) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_state_show(port: u16, state: *mut u32) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_ver_show(port: u16, ver: *mut u32) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_wd_reset(port: u16) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_wd_timeout_show(port: u16, wd_timeout: *mut u32) -> c_int;
	pub fn rte_pmd_ixgbe_bypass_wd_timeout_store(port: u16, timeout: u32) -> c_int;
	pub fn rte_pmd_ixgbe_macsec_config_rxsc(port: u16, mac: *mut u8, pi: u16) -> c_int;
	pub fn rte_pmd_ixgbe_macsec_config_txsc(port: u16, mac: *mut u8) -> c_int;
	pub fn rte_pmd_ixgbe_macsec_disable(port: u16) -> c_int;
	pub fn rte_pmd_ixgbe_macsec_enable(port: u16, en: u8, rp: u8) -> c_int;
	pub fn rte_pmd_ixgbe_macsec_select_rxsa(port: u16, idx: u8, an: u8, pn: u32, key: *mut u8) -> c_int;
	pub fn rte_pmd_ixgbe_macsec_select_txsa(port: u16, idx: u8, an: u8, pn: u32, key: *mut u8) -> c_int;
	pub fn rte_pmd_ixgbe_ping_vf(port: u16, vf: u16) -> c_int;
	pub fn rte_pmd_ixgbe_set_all_queues_drop_en(port: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_tc_bw_alloc(port: u16, tc_num: u8, bw_weight: *mut u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_tx_loopback(port: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_mac_addr(port: u16, vf: u16, mac_addr: *mut ether_addr) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_mac_anti_spoof(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_rate_limit(port: u16, vf: u16, tx_rate: u16, q_msk: u64) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_rx(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_rxmode(port: u16, vf: u16, rx_mask: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_split_drop_en(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_tx(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_vlan_anti_spoof(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_vlan_filter(port: u16, vlan: u16, vf_mask: u64, vlan_on: u8) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_vlan_insert(port: u16, vf: u16, vlan_id: u16) -> c_int;
	pub fn rte_pmd_ixgbe_set_vf_vlan_stripq(port: u16, vf: u16, on: u8) -> c_int;
	pub fn rte_pmd_softnic_run(port_id: u16) -> c_int;
}
