use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "20" x = "2" height = "14" y = "3" ></ rect > < line y2 = "21" x2 = "16" x1 = "8" y1 = "21" ></ line > < line x2 = "12" x1 = "12" y1 = "17" y2 = "21" ></ line > < / > } } pub const LucideMonitor : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;