use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 2v2" ></ path > < path d = "M5 2v2" ></ path > < path d = "M5 3H4a2 2 0 0 0-2 2v4a6 6 0 0 0 12 0V5a2 2 0 0 0-2-2h-1" ></ path > < path d = "M8 15a6 6 0 0 0 12 0v-3" ></ path > < circle r = "2" cx = "20" cy = "10" ></ circle > < / > } } pub const LUCIDE_STETHOSCOPE : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;