use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 2v2" ></ path > < path d = "M5 2v2" ></ path > < path d = "M5 3H4a2 2 0 0 0-2 2v4a6 6 0 0 0 12 0V5a2 2 0 0 0-2-2h-1" ></ path > < path d = "M8 15a6 6 0 0 0 12 0v-3" ></ path > < circle cx = "20" r = "2" cy = "10" ></ circle > < / > } } pub const LUCIDE_STETHOSCOPE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2")] } ;