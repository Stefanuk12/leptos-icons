use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" y = "3" x = "3" rx = "2" ></ rect > < line y1 = "15" y2 = "9" x2 = "15" x1 = "9" ></ line > < / > } } pub const LUCIDE_SQUARE_SLASH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;