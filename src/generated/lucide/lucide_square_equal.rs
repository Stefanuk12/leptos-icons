use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" rx = "2" y = "3" x = "3" ></ rect > < path d = "M7 10h10" ></ path > < path d = "M7 14h10" ></ path > < / > } } pub const LUCIDE_SQUARE_EQUAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;