use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" y = "3" rx = "2" height = "18" ></ rect > < line x2 = "15" x1 = "9" y1 = "15" y2 = "9" ></ line > < / > } } pub const LUCIDE_SQUARE_SLASH : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;