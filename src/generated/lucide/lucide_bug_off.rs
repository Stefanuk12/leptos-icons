use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 7.13V6a3 3 0 0 0-5.14-2.1L8 2" ></ path > < path d = "M14.12 3.88 16 2" ></ path > < path d = "M22 13h-4v-2a4 4 0 0 0-4-4h-1.3" ></ path > < path d = "M20.97 5c0 2.1-1.6 3.8-3.5 4" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M7.7 7.7A4 4 0 0 0 6 11v3a6 6 0 0 0 11.13 3.13" ></ path > < path d = "M12 20v-8" ></ path > < path d = "M6 13H2" ></ path > < path d = "M3 21c0-2.1 1.7-3.9 3.8-4" ></ path > < / > } } pub const LUCIDE_BUG_OFF : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;