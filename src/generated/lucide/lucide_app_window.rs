use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" rx = "2" y = "4" x = "2" height = "16" ></ rect > < path d = "M10 4v4" ></ path > < path d = "M2 8h20" ></ path > < path d = "M6 4v4" ></ path > < / > } } pub const LUCIDE_APP_WINDOW : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;