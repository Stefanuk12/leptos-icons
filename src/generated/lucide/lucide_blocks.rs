use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" rx = "1" x = "14" y = "3" height = "7" ></ rect > < path d = "M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" ></ path > < / > } } pub const LUCIDE_BLOCKS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none")] } ;