use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" ry = "2" x = "3" height = "18" width = "18" ></ rect > < line x2 = "21" x1 = "3" y2 = "9" y1 = "9" ></ line > < line x1 = "3" x2 = "21" y2 = "15" y1 = "15" ></ line > < line y1 = "9" x1 = "9" y2 = "21" x2 = "9" ></ line > < line y2 = "21" y1 = "9" x1 = "15" x2 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;