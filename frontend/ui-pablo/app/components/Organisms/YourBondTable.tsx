import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Typography,
  Tooltip,
} from "@mui/material";
import Image from "next/image";
import { BaseAsset, PairAsset } from "../Atoms";
import { useRouter } from "next/router";
import React from "react";
import { InfoOutlined } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import useStore from "../../store/useStore";

const tableHeaders: TableHeader[] = [
  {
    header: "Asset",
  },
  {
    header: "Claimable",
    tooltip: "Claimable",
  },
  {
    header: "Pending",
    tooltip: "Pending",
  },
  {
    header: "Vesting time",
    tooltip: "Vesting time",
  },
];

export const YourBondTable: React.FC = () => {
  const { activeBonds } = useStore();
  const router = useRouter();

  const handleRowClick = (offerId: number) => {
    router.push(`/bond/select/${offerId}`);
  };

  if (activeBonds.length == 0) {
    return (
      <Box textAlign="center" mt={3}>
        <Image
          src="/static/lemonade.png"
          css={{ mixBlendMode: "luminosity" }}
          width="96"
          height="96"
          alt="lemonade"
        />
        <Typography variant="body2" paddingTop={4} color="text.secondary">
          You currently do not have any active bonds.
        </Typography>
      </Box>
    );
  } else {
    return (
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              {tableHeaders.map((th) => (
                <TableCell align="left" key={th.header}>
                  <Box display="flex" alignItems="center" gap={1}>
                    {th.header}
                    {th.tooltip && (
                      <Tooltip arrow title={th.tooltip}>
                        <InfoOutlined color="primary" fontSize="small" />
                      </Tooltip>
                    )}
                  </Box>
                </TableCell>
              ))}
            </TableRow>
          </TableHead>
          <TableBody>
            {activeBonds.map((bond, index) => (
              <TableRow
                onClick={() => handleRowClick(bond.offerId)}
                key={index}
                sx={{ cursor: "pointer" }}
              >
                <TableCell align="left">
                  <BaseAsset label={bond.asset.symbol} icon={bond.asset.icon} />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    {bond.claimableAmount.toFormat()} CHAOS
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    {bond.pendingAmount.toFormat()} CHAOS
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">{bond.vestingTime}</Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }
  return null;
};