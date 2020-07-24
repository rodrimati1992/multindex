#[doc(hidden)]
#[macro_export]
macro_rules! new_IndexArgumentsAndStats{
    (@initialize
        prenorm = $prenorm:ident,
        ind_args = $ind_args:ident$(,)*
        error_handling(|$err:ident| $err_handling:expr ),
    )=>{{
        use $crate::pmr::{IndexArgument, IndexArgumentStats, IndexArgumentsAndStats};

        let mut stats = IndexArgumentStats::NEW;

        $crate::for_range!{ i in 0..$prenorm.len() =>
            let (elem, nstats) = match IndexArgument::from_prenorm(&$prenorm, i as _, stats) {
                Ok(x)=>x,
                Err($err)=>$err_handling,
            };
            stats = nstats;
            $ind_args[i] = elem;
        }

        IndexArgumentsAndStats{ $ind_args, stats }
    }};
    (@from_index_macro; $($index:expr),* $(,)?)=>{{
        const __PRENORM: &[$crate::pmr::PrenormIndex] =
            &$crate::prenorm_indices_from!($($index,)*);

        let mut ind_args = [$crate::pmr::IndexArgument::EMPTY; __PRENORM.len()];
        let mut ret_err = None;

        let iaas = $crate::new_IndexArgumentsAndStats!{
            @initialize
            prenorm = __PRENORM,
            ind_args = ind_args,
            error_handling(|e|{
                ret_err = $crate::pmr::Some(e);
                break;
            }),
        };

        $crate::pmr::ComputedConstants{
            err: ret_err,
            stats: iaas.stats,
            ind_args: iaas.ind_args,
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! prenorm_indices_from{
    ($($index:expr),* $(,)? )=>{
        [
            $( $crate::pmr::IntoPrenormIndex($index).call(), )*
        ]
    }
}
