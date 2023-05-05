--
-- Name: t_ws_quote; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.t_ws_quote (
                                   id bigint NOT NULL,
                                   dtg timestamp without time zone,
                                   dtg_updated timestamp without time zone,
                                   event character varying,
                                   symbol character varying,
                                   exchange_bid integer,
                                   price_bid numeric(20,10),
                                   size_bid integer,
                                   exchange_ask integer,
                                   price_ask numeric(20,10),
                                   size_ask integer
);


ALTER TABLE public.t_ws_quote OWNER TO postgres;

--
-- Name: t_ws_quote_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE IF NOT EXISTS public.t_ws_quote_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.t_ws_quote_id_seq OWNER TO postgres;

--
-- Name: t_ws_quote_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.t_ws_quote_id_seq OWNED BY public.t_ws_quote.id;

ALTER TABLE ONLY public.t_ws_quote
    ALTER COLUMN id SET DEFAULT nextval('public.t_ws_quote_id_seq'::regclass);

--
-- Name: t_ws_quote t_ws_quote_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.t_ws_quote DROP CONSTRAINT IF EXISTS t_ws_quote_pkey;
ALTER TABLE ONLY public.t_ws_quote
    ADD CONSTRAINT t_ws_quote_pkey PRIMARY KEY (id);