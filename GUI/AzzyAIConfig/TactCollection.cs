// TactCollection.cs
//
// Programmed by Machiavellian of iRO Chaos
//
// Description:
// This file contains code for the TactCollection class, which implements
// the generic interface ICollection<T>.

using System;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections;
using System.Collections.Generic;


namespace AzzyAIConfig
{
    class TactCollection : System.Collections.Generic.ICollection<Tact>
    {
        Tact[] _items = new Tact[0];

        public TactCollection()
        {
        }

        public TactCollection(Tact[] items)
        {
            // Set the items array
            _items = items;
        }

        public TactCollection(Tact[] items, bool readOnly)
        {
            // Set the items array
            _items = items;

            // Set the collection as read only
            _readOnly = readOnly;
        }

        public void Add(Tact item)
        {
            // Check if the collection is read only
            if (_readOnly)
            {
                // Throw a new exception
                throw new Exception("The collection is read-only.");
            }
            // If the collection is not read only
            else
            {
                // Create a new tactics array
                Tact[] old = new Tact[_items.Length];

                // Copy the old tactics array to the new one
                _items.CopyTo(old, 0);

                // Create a new tactics array with one more slot than the old one
                _items = new Tact[old.Length + 1];

                // Copy the old tactics array to the new one
                old.CopyTo(_items, 0);

                // Set the last slot of the new array to the new tactic
                _items[old.Length] = item;

                // Sort the array
                Array.Sort(_items);
            }
        }

        public void Clear()
        {
            // Check if the collection is read only
            if (_readOnly)
            {
                // Throw a new exception
                throw new Exception("The collection is read-only.");
            }
            // If the collection is not read only
            else
            {
                // Clear the collection array
                _items = new Tact[0];
            }
        }

        public bool Contains(Tact item)
        {
            // Check if the collection array has more than one item
            if (_items.Length > 0)
            {
                // Create a new tactics array
                Tact[] search = new Tact[_items.Length];

                // Copy the old tactics array to the new one
                _items.CopyTo(search, 0);

                // Run through the tactics in the new array
                foreach (Tact t in search)
                {
                    // Check if the current tactic equals item
                    if (t == item)
                    {
                        // Return true
                        return true;
                    }
                }
            }

            // Return false
            return false;
        }

        public void CopyTo(Tact[] array, int arrayIndex)
        {
            // Cope the collection array to array at arrayIndex
            _items.CopyTo(array, arrayIndex);
        }

        public int Count
        {
            get { return _items.Length; }
        }

        bool _readOnly = false;
        public bool IsReadOnly
        {
            get { return _readOnly; }
        }

        public bool Remove(Tact item)
        {
            // Check if the collection is read only
            if (_readOnly)
            {
                // Throw a new exception
                throw new Exception("The collection is read-only.");
            }
            // If the collection is not read only and contains item
            else if (Contains(item))
            {
                // Find the item
                int index = 0;
                while (_items[index] != item) index++;

                // Keep the original tactic objects: the list box edits them directly
                Tact[] kept = new Tact[_items.Length - 1];
                Array.Copy(_items, 0, kept, 0, index);
                Array.Copy(_items, index + 1, kept, index, _items.Length - index - 1);
                _items = kept;

                // Sort the new array
                Array.Sort(_items);

                // Return true
                return true;
            }

            // Return false
            return false;
        }

        public IEnumerator<Tact> GetEnumerator()
        {
            // Return the enumerator for this collection
            return new TactEnumerator(_items);
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            throw new NotImplementedException();
        }
    }
}