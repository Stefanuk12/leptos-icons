use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" x = "3" width = "18" height = "18" ></ rect > < path d = "m8 14 4-4 4 4" ></ path > < / > } } pub const LUCIDE_SQUARE_CHEVRON_UP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;