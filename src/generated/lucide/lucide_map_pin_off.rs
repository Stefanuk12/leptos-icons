use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.43 5.43A8.06 8.06 0 0 0 4 10c0 6 8 12 8 12a29.94 29.94 0 0 0 5-5" ></ path > < path d = "M19.18 13.52A8.66 8.66 0 0 0 20 10a8 8 0 0 0-8-8 7.88 7.88 0 0 0-3.52.82" ></ path > < path d = "M9.13 9.13A2.78 2.78 0 0 0 9 10a3 3 0 0 0 3 3 2.78 2.78 0 0 0 .87-.13" ></ path > < path d = "M14.9 9.25a3 3 0 0 0-2.15-2.16" ></ path > < line x1 = "2" x2 = "22" y1 = "2" y2 = "22" ></ line > < / > } } pub const LUCIDE_MAP_PIN_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;