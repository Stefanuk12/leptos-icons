use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" y = "3" ry = "2" x = "3" height = "18" ></ rect > < line y1 = "9" x2 = "21" y2 = "9" x1 = "3" ></ line > < line y2 = "15" x2 = "21" y1 = "15" x1 = "3" ></ line > < line y1 = "9" y2 = "21" x1 = "9" x2 = "9" ></ line > < line y2 = "21" x1 = "15" y1 = "9" x2 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;