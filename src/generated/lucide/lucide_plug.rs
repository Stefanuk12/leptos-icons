use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22v-5" ></ path > < path d = "M9 8V2" ></ path > < path d = "M15 8V2" ></ path > < path d = "M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z" ></ path > < / > } } pub const LUCIDE_PLUG : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;