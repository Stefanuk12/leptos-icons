use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" y = "4" height = "16" rx = "2" ></ rect > < path d = "M10 4v4" ></ path > < path d = "M2 8h20" ></ path > < path d = "M6 4v4" ></ path > < / > } } pub const LUCIDE_APP_WINDOW : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;