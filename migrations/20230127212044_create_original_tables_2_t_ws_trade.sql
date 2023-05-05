--
-- Name: t_ws_trade; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.t_ws_trade (
                                   id bigint NOT NULL,
                                   dtg timestamp without time zone,
                                   dtg_updated timestamp without time zone,
                                   event character varying,
                                   symbol character varying,
                                   id_trade numeric(20,10),
                                   exchange integer,
                                   price numeric(20,10),
                                   size integer,
                                   id_tape bigint NOT NULL
);


ALTER TABLE public.t_ws_trade OWNER TO postgres;

--
-- Name: t_ws_trade_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE IF NOT EXISTS public.t_ws_trade_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ws_trade_id_seq OWNER TO postgres;

--
-- Name: t_ws_trade_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_ws_trade_id_seq OWNED BY public.t_ws_trade.id;


--
-- Name: t_ws_trade_id_tape_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE IF NOT EXISTS public.t_ws_trade_id_tape_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ws_trade_id_tape_seq OWNER TO postgres;

--
-- Name: t_ws_trade_id_tape_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_ws_trade_id_tape_seq OWNED BY public.t_ws_trade.id_tape;