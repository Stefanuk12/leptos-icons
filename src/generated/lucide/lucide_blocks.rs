use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "14" rx = "1" height = "7" width = "7" ></ rect > < path d = "M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" ></ path > < / > } } pub const LUCIDE_BLOCKS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;