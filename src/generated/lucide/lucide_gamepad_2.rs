use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y1 = "11" x2 = "10" y2 = "11" ></ line > < line x2 = "8" y2 = "13" x1 = "8" y1 = "9" ></ line > < line y1 = "12" x1 = "15" y2 = "12" x2 = "15.01" ></ line > < line y2 = "10" x1 = "18" x2 = "18.01" y1 = "10" ></ line > < path d = "M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z" ></ path > < / > } } pub const LUCIDE_GAMEPAD_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2")] } ;