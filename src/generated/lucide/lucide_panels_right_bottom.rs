use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" y = "3" rx = "2" width = "18" ></ rect > < path d = "M3 15h12" ></ path > < path d = "M15 3v18" ></ path > < / > } } pub const LUCIDE_PANELS_RIGHT_BOTTOM : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;