use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "12" cx = "12" ></ circle > < path d = "M12 16.5A4.5 4.5 0 1 1 7.5 12 4.5 4.5 0 1 1 12 7.5a4.5 4.5 0 1 1 4.5 4.5 4.5 4.5 0 1 1-4.5 4.5" ></ path > < path d = "M12 7.5V9" ></ path > < path d = "M7.5 12H9" ></ path > < path d = "M16.5 12H15" ></ path > < path d = "M12 16.5V15" ></ path > < path d = "m8 8 1.88 1.88" ></ path > < path d = "M14.12 9.88 16 8" ></ path > < path d = "m8 16 1.88-1.88" ></ path > < path d = "M14.12 14.12 16 16" ></ path > < / > } } pub const LUCIDE_FLOWER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;