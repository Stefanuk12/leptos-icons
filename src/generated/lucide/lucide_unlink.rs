use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71" ></ path > < path d = "m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71" ></ path > < line x2 = "8" y1 = "2" y2 = "5" x1 = "8" ></ line > < line y1 = "8" y2 = "8" x2 = "5" x1 = "2" ></ line > < line x2 = "16" y2 = "22" x1 = "16" y1 = "19" ></ line > < line y1 = "16" y2 = "16" x2 = "22" x1 = "19" ></ line > < / > } } pub const LucideUnlink : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;