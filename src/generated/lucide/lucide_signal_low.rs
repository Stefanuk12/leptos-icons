use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20h.01" ></ path > < path d = "M7 20v-4" ></ path > < / > } } pub const LUCIDE_SIGNAL_LOW : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;