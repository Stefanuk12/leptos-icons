use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" height = "14" x = "2" width = "20" ></ rect > < line y1 = "21" y2 = "21" x1 = "8" x2 = "16" ></ line > < line y2 = "21" x2 = "12" y1 = "17" x1 = "12" ></ line > < / > } } pub const LucideMonitor : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;