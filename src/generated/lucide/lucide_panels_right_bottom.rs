use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" rx = "2" y = "3" x = "3" ></ rect > < path d = "M3 15h12" ></ path > < path d = "M15 3v18" ></ path > < / > } } pub const LUCIDE_PANELS_RIGHT_BOTTOM : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;