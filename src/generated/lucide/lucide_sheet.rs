use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" ry = "2" y = "3" width = "18" height = "18" rx = "2" ></ rect > < line x2 = "21" y1 = "9" x1 = "3" y2 = "9" ></ line > < line x1 = "3" x2 = "21" y2 = "15" y1 = "15" ></ line > < line x2 = "9" y2 = "21" x1 = "9" y1 = "9" ></ line > < line y2 = "21" y1 = "9" x1 = "15" x2 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;