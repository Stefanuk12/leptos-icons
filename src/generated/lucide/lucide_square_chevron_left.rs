use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" y = "3" x = "3" width = "18" ></ rect > < path d = "m14 16-4-4 4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;