use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" width = "18" y = "3" x = "3" ></ rect > < path d = "M3 15h12" ></ path > < path d = "M15 3v18" ></ path > < / > } } pub const LUCIDE_PANELS_RIGHT_BOTTOM : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;