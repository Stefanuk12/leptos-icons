use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "16" x = "2" y = "4" rx = "2" ></ rect > < path d = "M10 4v4" ></ path > < path d = "M2 8h20" ></ path > < path d = "M6 4v4" ></ path > < / > } } pub const LUCIDE_APP_WINDOW : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;