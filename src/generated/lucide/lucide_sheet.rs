use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" width = "18" rx = "2" ry = "2" height = "18" ></ rect > < line y2 = "9" x1 = "3" x2 = "21" y1 = "9" ></ line > < line y1 = "15" y2 = "15" x2 = "21" x1 = "3" ></ line > < line y2 = "21" x1 = "9" x2 = "9" y1 = "9" ></ line > < line x2 = "15" y2 = "21" x1 = "15" y1 = "9" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;