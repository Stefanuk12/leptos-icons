use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9.5 8 12l2 2.5" ></ path > < path d = "m14 9.5 2 2.5-2 2.5" ></ path > < rect y = "3" width = "18" x = "3" rx = "2" height = "18" ></ rect > < / > } } pub const LUCIDE_SQUARE_CODE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;