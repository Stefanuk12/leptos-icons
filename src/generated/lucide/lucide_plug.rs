use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22v-5" ></ path > < path d = "M9 8V2" ></ path > < path d = "M15 8V2" ></ path > < path d = "M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z" ></ path > < / > } } pub const LUCIDE_PLUG : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;