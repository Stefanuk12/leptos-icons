use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" rx = "2" x = "3" y = "3" ></ rect > < line y2 = "9" y1 = "15" x2 = "15" x1 = "9" ></ line > < / > } } pub const LUCIDE_SQUARE_SLASH : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;