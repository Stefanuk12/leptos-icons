use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4" ></ path > < path d = "M14 13.12c0 2.38 0 6.38-1 8.88" ></ path > < path d = "M17.29 21.02c.12-.6.43-2.3.5-3.02" ></ path > < path d = "M2 12a10 10 0 0 1 18-6" ></ path > < path d = "M2 16h.01" ></ path > < path d = "M21.8 16c.2-2 .131-5.354 0-6" ></ path > < path d = "M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2" ></ path > < path d = "M8.65 22c.21-.66.45-1.32.57-2" ></ path > < path d = "M9 6.8a6 6 0 0 1 9 5.2v2" ></ path > < / > } } pub const LUCIDE_FINGERPRINT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;