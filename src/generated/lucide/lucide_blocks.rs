use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "7" rx = "1" width = "7" x = "14" y = "3" ></ rect > < path d = "M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" ></ path > < / > } } pub const LUCIDE_BLOCKS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24")] } ;