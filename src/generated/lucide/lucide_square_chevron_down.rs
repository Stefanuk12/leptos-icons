use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" rx = "2" height = "18" x = "3" ></ rect > < path d = "m16 10-4 4-4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_DOWN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;