use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" x1 = "2" x2 = "22" y1 = "2" ></ line > < path d = "M18.89 13.23A7.12 7.12 0 0 0 19 12v-2" ></ path > < path d = "M5 10v2a7 7 0 0 0 12 5" ></ path > < path d = "M15 9.34V5a3 3 0 0 0-5.68-1.33" ></ path > < path d = "M9 9v3a3 3 0 0 0 5.12 2.12" ></ path > < line x2 = "12" x1 = "12" y1 = "19" y2 = "22" ></ line > < / > } } pub const LUCIDE_MIC_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;