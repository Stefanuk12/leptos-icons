use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "2" height = "18" ry = "2" width = "18" ></ rect > < line y1 = "9" x1 = "3" y2 = "9" x2 = "21" ></ line > < line y2 = "15" x1 = "3" x2 = "21" y1 = "15" ></ line > < line y2 = "21" x2 = "9" x1 = "9" y1 = "9" ></ line > < line y1 = "9" y2 = "21" x2 = "15" x1 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;