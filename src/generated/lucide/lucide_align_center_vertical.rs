use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v20" ></ path > < path d = "M8 10H4a2 2 0 0 1-2-2V6c0-1.1.9-2 2-2h4" ></ path > < path d = "M16 10h4a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-4" ></ path > < path d = "M8 20H7a2 2 0 0 1-2-2v-2c0-1.1.9-2 2-2h1" ></ path > < path d = "M16 14h1a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-1" ></ path > < / > } } pub const LUCIDE_ALIGN_CENTER_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;