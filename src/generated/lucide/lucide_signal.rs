use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20h.01" ></ path > < path d = "M7 20v-4" ></ path > < path d = "M12 20v-8" ></ path > < path d = "M17 20V8" ></ path > < path d = "M22 4v16" ></ path > < / > } } pub const LUCIDE_SIGNAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;