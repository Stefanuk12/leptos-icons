use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 20v-8a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v8" ></ path > < path d = "M5 10V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v4" ></ path > < path d = "M3 18h18" ></ path > < / > } } pub const LUCIDE_BED_SINGLE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;