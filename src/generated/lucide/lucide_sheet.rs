use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" ry = "2" x = "3" y = "3" height = "18" ></ rect > < line y2 = "9" x1 = "3" x2 = "21" y1 = "9" ></ line > < line x2 = "21" y1 = "15" x1 = "3" y2 = "15" ></ line > < line x2 = "9" x1 = "9" y1 = "9" y2 = "21" ></ line > < line x1 = "15" y2 = "21" y1 = "9" x2 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;