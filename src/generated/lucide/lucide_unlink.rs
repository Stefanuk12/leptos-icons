use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71" ></ path > < path d = "m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71" ></ path > < line x2 = "8" y2 = "5" y1 = "2" x1 = "8" ></ line > < line x2 = "5" x1 = "2" y2 = "8" y1 = "8" ></ line > < line x2 = "16" x1 = "16" y1 = "19" y2 = "22" ></ line > < line y1 = "16" x1 = "19" x2 = "22" y2 = "16" ></ line > < / > } } pub const LUCIDE_UNLINK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2")] } ;