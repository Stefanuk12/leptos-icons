use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" y = "3" height = "18" width = "18" ></ rect > < path d = "M17 12h-2l-2 5-2-10-2 5H7" ></ path > < / > } } pub const LUCIDE_SQUARE_ACTIVITY : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;