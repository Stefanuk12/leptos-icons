use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 20h.01" ></ path > < path d = "M7 20v-4" ></ path > < / > } } pub const LUCIDE_SIGNAL_LOW : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;