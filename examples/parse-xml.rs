fn main() {
    const DATA: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <data xmlns="event">
      <event id="0">
        <start_date><![CDATA[2021-03-23 07:45]]></start_date>
        <end_date><![CDATA[2021-03-23 08:30]]></end_date>
        <text><![CDATA[EW-3bNP-Re]]></text>
        <kommentar><![CDATA[GDP & Economic Growth]]></kommentar>
        <klasse><![CDATA[3bNP]]></klasse>
        <anzeige><![CDATA[]]></anzeige>
        <rasterid_wp><![CDATA[]]></rasterid_wp>
        <readonly><![CDATA[1]]></readonly>
        <zimmer><![CDATA[]]></zimmer>
        <lehrer><![CDATA[-1]]></lehrer>
        <lehrer_kurs><![CDATA[]]></lehrer_kurs>
        <lehrer_rv><![CDATA[]]></lehrer_rv>
        <reservator><![CDATA[]]></reservator>
        <reservation><![CDATA[]]></reservation>
        <istrv><![CDATA[]]></istrv>
        <markierung><![CDATA[]]></markierung>
        <textdeco><![CDATA[]]></textdeco>
        <color><![CDATA[#B34C27]]></color>
        <event_type><![CDATA[1_pru]]></event_type>
        <show_pru_link><![CDATA[]]></show_pru_link>
        <show_abs_link><![CDATA[]]></show_abs_link>
        <show_haa_link><![CDATA[]]></show_haa_link>
        <klassentyp><![CDATA[]]></klassentyp>
        <lehrerkuerzel><![CDATA[]]></lehrerkuerzel>
        <kurskuerzel><![CDATA[]]></kurskuerzel>
        <kursid><![CDATA[]]></kursid>
        <kursind><![CDATA[]]></kursind>
        <neuerlehrer><![CDATA[]]></neuerlehrer>
        <mandant><![CDATA[]]></mandant>
        <mandantname><![CDATA[]]></mandantname>
        <info1><![CDATA[22.02.2021]]></info1>
        <anzeigestp><![CDATA[]]></anzeigestp>
        <fachkuerzel><![CDATA[]]></fachkuerzel>
        <history><![CDATA[]]></history>
      </event>
      <event id="1">
        <start_date><![CDATA[2021-03-22 11:20]]></start_date>
        <end_date><![CDATA[2021-03-22 12:05]]></end_date>
        <text><![CDATA[P-3bNP-Mü]]></text>
        <kommentar><![CDATA[Prüfung 4]]></kommentar>
        <klasse><![CDATA[3bNP]]></klasse>
        <anzeige><![CDATA[]]></anzeige>
        <rasterid_wp><![CDATA[]]></rasterid_wp>
        <readonly><![CDATA[1]]></readonly>
        <zimmer><![CDATA[]]></zimmer>
        <lehrer><![CDATA[-1]]></lehrer>
        <lehrer_kurs><![CDATA[]]></lehrer_kurs>
        <lehrer_rv><![CDATA[]]></lehrer_rv>
        <reservator><![CDATA[]]></reservator>
        <reservation><![CDATA[]]></reservation>
        <istrv><![CDATA[]]></istrv>
        <markierung><![CDATA[]]></markierung>
        <textdeco><![CDATA[]]></textdeco>
        <color><![CDATA[#B34C27]]></color>
        <event_type><![CDATA[1_pru]]></event_type>
        <show_pru_link><![CDATA[]]></show_pru_link>
        <show_abs_link><![CDATA[]]></show_abs_link>
        <show_haa_link><![CDATA[]]></show_haa_link>
        <klassentyp><![CDATA[]]></klassentyp>
        <lehrerkuerzel><![CDATA[]]></lehrerkuerzel>
        <kurskuerzel><![CDATA[]]></kurskuerzel>
        <kursid><![CDATA[]]></kursid>
        <kursind><![CDATA[]]></kursind>
        <neuerlehrer><![CDATA[]]></neuerlehrer>
        <mandant><![CDATA[]]></mandant>
        <mandantname><![CDATA[]]></mandantname>
        <info1><![CDATA[04.02.2021]]></info1>
        <anzeigestp><![CDATA[]]></anzeigestp>
        <fachkuerzel><![CDATA[]]></fachkuerzel>
        <history><![CDATA[]]></history>
      </event>
      <event id="2">
        <start_date><![CDATA[2021-03-26 09:40]]></start_date>
        <end_date><![CDATA[2021-03-26 10:25]]></end_date>
        <text><![CDATA[sM-3bNP,3NP-Wr]]></text>
        <kommentar><![CDATA[Prüfung 3]]></kommentar>
        <klasse><![CDATA[3bNP,3NP]]></klasse>
        <anzeige><![CDATA[]]></anzeige>
        <rasterid_wp><![CDATA[]]></rasterid_wp>
        <readonly><![CDATA[1]]></readonly>
        <zimmer><![CDATA[]]></zimmer>
        <lehrer><![CDATA[-1]]></lehrer>
        <lehrer_kurs><![CDATA[]]></lehrer_kurs>
        <lehrer_rv><![CDATA[]]></lehrer_rv>
        <reservator><![CDATA[]]></reservator>
        <reservation><![CDATA[]]></reservation>
        <istrv><![CDATA[]]></istrv>
        <markierung><![CDATA[]]></markierung>
        <textdeco><![CDATA[]]></textdeco>
        <color><![CDATA[#B34C27]]></color>
        <event_type><![CDATA[1_pru]]></event_type>
        <show_pru_link><![CDATA[]]></show_pru_link>
        <show_abs_link><![CDATA[]]></show_abs_link>
        <show_haa_link><![CDATA[]]></show_haa_link>
        <klassentyp><![CDATA[]]></klassentyp>
        <lehrerkuerzel><![CDATA[]]></lehrerkuerzel>
        <kurskuerzel><![CDATA[]]></kurskuerzel>
        <kursid><![CDATA[]]></kursid>
        <kursind><![CDATA[]]></kursind>
        <neuerlehrer><![CDATA[]]></neuerlehrer>
        <mandant><![CDATA[]]></mandant>
        <mandantname><![CDATA[]]></mandantname>
        <info1><![CDATA[31.01.2021]]></info1>
        <anzeigestp><![CDATA[]]></anzeigestp>
        <fachkuerzel><![CDATA[]]></fachkuerzel>
        <history><![CDATA[]]></history>
      </event>
    </data>
    "#;

    let root: minidom::Element = DATA.parse().unwrap();
    assert!(
        "2021-03-23 07:45"
            == root
                .get_child("event", "event")
                .unwrap()
                .get_child("start_date", "event")
                .unwrap()
                .text()
    );
}
