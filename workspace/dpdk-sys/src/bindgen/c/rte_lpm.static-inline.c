#include <rte_lpm.h>
#include "bindgen/c/rte_lpm.static-inline.h"

void rust_rte_lpm_lookupx4(const struct rte_lpm * lpm, xmm_t ip, uint32_t hop[4], uint32_t defv)
{
	rte_lpm_lookupx4(lpm, ip, hop, defv);
}

int rust_rte_lpm_lookup_bulk_func(const struct rte_lpm * lpm, const uint32_t * ips, uint32_t * next_hops, const unsigned n)
{
	return rte_lpm_lookup_bulk_func(lpm, ips, next_hops, n);
}

int rust_rte_lpm_lookup(struct rte_lpm * lpm, uint32_t ip, uint32_t * next_hop)
{
	return rte_lpm_lookup(lpm, ip, next_hop);
}
