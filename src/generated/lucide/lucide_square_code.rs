use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "18" rx = "2" width = "18" x = "3" ></ rect > < path d = "m10 10-2 2 2 2" ></ path > < path d = "m14 14 2-2-2-2" ></ path > < / > } } pub const LUCIDE_SQUARE_CODE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;