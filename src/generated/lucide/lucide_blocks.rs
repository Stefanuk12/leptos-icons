use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "7" x = "14" rx = "1" height = "7" ></ rect > < path d = "M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" ></ path > < / > } } pub const LUCIDE_BLOCKS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round")] } ;