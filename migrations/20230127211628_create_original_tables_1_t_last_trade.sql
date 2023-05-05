-- t_last_trade

--
-- Name: t_last_trade; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.t_last_trade (
                                     id bigint NOT NULL,
                                     symbol character varying,
                                     dtg timestamp without time zone,
                                     dtg_updated timestamp without time zone,
                                     price numeric(20,10),
                                     size numeric(20,10),
                                     exchange integer,
                                     cond1 integer,
                                     cond2 integer,
                                     cond3 integer,
                                     cond4 integer
);


ALTER TABLE public.t_last_trade OWNER TO postgres;

--
-- Name: t_last_trade_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE IF NOT EXISTS public.t_last_trade_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_last_trade_id_seq OWNER TO postgres;

--
-- Name: t_last_trade_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_last_trade_id_seq OWNED BY public.t_last_trade.id;
